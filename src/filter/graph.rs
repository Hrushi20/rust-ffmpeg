use std::mem::MaybeUninit;
use std::{mem, ptr};

use super::{Context};
// use super::{Context, Filter};
use Error;
use filter::types::{AVFilterContext, AVFilterGraph, AVFilterInOut};
use avfilter_wasmedge;

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

    // pub fn add<'a, 'b>(
    //     &'a mut self,
    //     filter: &Filter,
    //     name: &str,
    //     args: &str,
    // ) -> Result<Context<'b>, Error>
    // where
    //     'a: 'b,
    // {
    //     unsafe {
    //         let name = CString::new(name).unwrap();
    //         let args = CString::new(args).unwrap();
    //         let mut context = ptr::null_mut();
    //
    //         match avfilter_graph_create_filter(
    //             &mut context as *mut *mut AVFilterContext,
    //             filter.as_ptr(),
    //             name.as_ptr(),
    //             args.as_ptr(),
    //             ptr::null_mut(),
    //             self.as_mut_ptr(),
    //         ) {
    //             n if n >= 0 => Ok(Context::wrap(context)),
    //             e => Err(Error::from(e)),
    //         }
    //     }
    // }

    pub fn get<'a, 'b>(&'b mut self, name: &str) -> Option<Context<'b>>
    where
        'a: 'b,
    {
        unsafe {
            let avfilter_context = MaybeUninit::<AVFilterContext>::uninit();
            avfilter_wasmedge::avfilter_graph_get_filter(avfilter_context.as_ptr() as u32,self.ptr(), name.as_ptr(),name.len());

            let avfilter_context = ptr::read(avfilter_context.as_ptr());
            if avfilter_context == 0 {
                None
            } else {
                Some(Context::wrap(avfilter_context))
            }
        }
    }

    // pub fn dump(&self) -> String {
    //     unsafe {
    //         let ptr = avfilter_graph_dump(self.as_ptr() as *mut _, ptr::null());
    //         let cstr = from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes());
    //         let string = cstr.to_owned();
    //
    //         av_free(ptr as *mut _);
    //
    //         string
    //     }
    // }

    // pub fn input(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
    //     Parser::new(self).input(name, pad)
    // }
    //
    // pub fn output(&mut self, name: &str, pad: usize) -> Result<Parser, Error> {
    //     Parser::new(self).output(name, pad)
    // }

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

    // pub fn input(mut self, name: &str, pad: usize) -> Result<Self, Error> {
    //     unsafe {
    //         let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;
    //         let input = avfilter_inout_alloc();
    //
    //         if input.is_null() {
    //             panic!("out of memory");
    //         }
    //
    //         let name = CString::new(name).unwrap();
    //
    //         (*input).name = av_strdup(name.as_ptr());
    //         (*input).filter_ctx = context.as_mut_ptr();
    //         (*input).pad_idx = pad as c_int;
    //         (*input).next = ptr::null_mut();
    //
    //         if self.inputs.is_null() {
    //             self.inputs = input;
    //         } else {
    //             (*self.inputs).next = input;
    //         }
    //     }
    //
    //     Ok(self)
    // }
    //
    // pub fn output(mut self, name: &str, pad: usize) -> Result<Self, Error> {
    //     unsafe {
    //         let mut context = self.graph.get(name).ok_or(Error::InvalidData)?;
    //         let output = avfilter_inout_alloc();
    //
    //         if output.is_null() {
    //             panic!("out of memory");
    //         }
    //
    //         let name = CString::new(name).unwrap();
    //
    //         (*output).name = av_strdup(name.as_ptr());
    //         (*output).filter_ctx = context.as_mut_ptr();
    //         (*output).pad_idx = pad as c_int;
    //         (*output).next = ptr::null_mut();
    //
    //         if self.outputs.is_null() {
    //             self.outputs = output;
    //         } else {
    //             (*self.outputs).next = output;
    //         }
    //     }
    //
    //     Ok(self)
    // }

    pub fn parse(mut self, spec: &str) -> Result<(), Error> {
        unsafe {

            let result = avfilter_wasmedge::avfilter_graph_parse_ptr(
                self.graph.ptr(),
                spec.as_ptr(),
                spec.len(),
                self.inputs,
                self.outputs
            );

            // This may fail. Check once.
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
