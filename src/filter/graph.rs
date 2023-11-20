use std::mem::MaybeUninit;
use std::{mem, ptr};

use avfilter_wasmedge;
use filter::types::{AVFilterContext, AVFilterGraph, AVFilterInOut};
use Error;

use super::{Context, Filter};

pub struct Graph {
    ptr: AVFilterGraph,
}

unsafe impl Send for Graph {}

unsafe impl Sync for Graph {}

impl Graph {
    pub unsafe fn wrap(ptr: AVFilterGraph) -> Self {
        Graph { ptr }
    }

    pub unsafe fn ptr(&self) -> AVFilterGraph {
        self.ptr
    }
}

impl Graph {
    pub fn new() -> Self {
        unsafe {
            let avfilter_graph = MaybeUninit::<AVFilterGraph>::uninit();
            avfilter_wasmedge::avfilter_graph_alloc(avfilter_graph.as_ptr() as u32);

            let avfilter_graph = ptr::read(avfilter_graph.as_ptr());
            if avfilter_graph == 0 {
                panic!("out of memory");
            }

            Graph::wrap(avfilter_graph)
        }
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        unsafe {
            match avfilter_wasmedge::avfilter_graph_config(self.ptr()) {
                0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn add<'a, 'b>(
        &'a mut self,
        filter: &Filter,
        name: &str,
        args: &str,
    ) -> Result<Context<'b>, Error>
    where
        'a: 'b,
    {
        unsafe {
            let ctx = MaybeUninit::<AVFilterContext>::uninit();

            match avfilter_wasmedge::avfilter_graph_create_filter(
                ctx.as_ptr() as u32,
                filter.ptr(),
                name.as_ptr(),
                name.len(),
                args.as_ptr(),
                args.len(),
                self.ptr(),
            ) {
                n if n >= 0 => {
                    let ctx = ptr::read(ctx.as_ptr());
                    Ok(Context::wrap(ctx))
                }
                e => Err(Error::from(e)),
            }
        }
    }

    pub fn get<'a, 'b>(&'b mut self, name: &str) -> Option<Context<'b>>
    where
        'a: 'b,
    {
        unsafe {
            let avfilter_context = MaybeUninit::<AVFilterContext>::uninit();
            avfilter_wasmedge::avfilter_graph_get_filter(
                avfilter_context.as_ptr() as u32,
                self.ptr(),
                name.as_ptr(),
                name.len(),
            );

            let avfilter_context = ptr::read(avfilter_context.as_ptr());
            if avfilter_context == 0 {
                None
            } else {
                Some(Context::wrap(avfilter_context))
            }
        }
    }

    pub fn dump(&self) -> String {
        unsafe {
            let len = avfilter_wasmedge::avfilter_graph_dump_length(self.ptr()) as usize;
            let graph_str = vec![0u8; len];
            avfilter_wasmedge::avfilter_graph_dump(self.ptr(), graph_str.as_ptr(), len);
            avfilter_wasmedge::avfilter_free_graph_str(self.ptr());
            String::from_utf8_unchecked(graph_str)
        }
    }

    pub fn input(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
        Parser::new(self).input(name, pad)
    }

    pub fn output(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
        Parser::new(self).output(name, pad)
    }

    pub fn parse(&mut self, spec: &str) -> Result<(), Error> {
        Parser::new(self).parse(spec)
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            avfilter_wasmedge::avfilter_graph_free(self.ptr());
        }
    }
}

pub struct Parser<'a> {
    graph: &'a mut Graph,
    inputs: AVFilterInOut,
    outputs: AVFilterInOut,
}

impl<'a> Parser<'a> {
    pub fn new(graph: &mut Graph) -> Parser {
        unsafe {
            Parser {
                graph,
                inputs: mem::zeroed::<AVFilterInOut>(),
                outputs: mem::zeroed::<AVFilterInOut>(),
            }
        }
    }

    pub fn input(mut self, name: &str, pad: usize) -> Result<Self, Error> {
        unsafe {
            let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;

            let input = MaybeUninit::<AVFilterInOut>::uninit();
            avfilter_wasmedge::avfilter_inout_alloc(input.as_ptr() as u32);
            let input = ptr::read(input.as_ptr()) as u32;

            if input == 0 {
                panic!("out of memory");
            }

            avfilter_wasmedge::avfilter_inout_set_name(input, name.as_ptr(), name.len());
            avfilter_wasmedge::avfilter_inout_set_filter_ctx(input, context.ptr());
            avfilter_wasmedge::avfilter_inout_set_pad_idx(input, pad as i32);
            avfilter_wasmedge::avfilter_inout_set_next(input, 0); // Setting Null

            if self.inputs == 0 {
                self.inputs = input;
            } else {
                avfilter_wasmedge::avfilter_inout_set_next(self.inputs, input);
            }
        }

        Ok(self)
    }

    pub fn output(mut self, name: &str, pad: usize) -> Result<Self, Error> {
        unsafe {
            let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;

            let output = MaybeUninit::<AVFilterInOut>::uninit();
            avfilter_wasmedge::avfilter_inout_alloc(output.as_ptr() as u32);
            let output = ptr::read(output.as_ptr());

            if output == 0 {
                panic!("out of memory");
            }

            avfilter_wasmedge::avfilter_inout_set_name(output, name.as_ptr(), name.len());
            avfilter_wasmedge::avfilter_inout_set_filter_ctx(output, context.ptr());
            avfilter_wasmedge::avfilter_inout_set_pad_idx(output, pad as i32);
            avfilter_wasmedge::avfilter_inout_set_next(output, 0); // Setting Null

            if self.outputs == 0 {
                self.outputs = output;
            } else {
                avfilter_wasmedge::avfilter_inout_set_next(self.outputs, output);
            }
        }

        Ok(self)
    }

    pub fn parse(mut self, spec: &str) -> Result<(), Error> {
        unsafe {
            let result = avfilter_wasmedge::avfilter_graph_parse_ptr(
                self.graph.ptr(),
                spec.as_ptr(),
                spec.len(),
                self.inputs,
                self.outputs,
            );

            avfilter_wasmedge::avfilter_inout_free(self.inputs);
            avfilter_wasmedge::avfilter_inout_free(self.outputs);

            match result {
                n if n >= 0 => Ok(()),
                e => Err(Error::from(e)),
            }
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
