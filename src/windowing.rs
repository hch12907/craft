use glutin::{ Api as GlApi, GlProfile, GlRequest };
use glutin::{ Context, ContextBuilder, PossiblyCurrent };
use glutin::window::WindowBuilder;

pub use glutin::event::{ Event, WindowEvent };
pub use glutin::event_loop::{ ControlFlow, EventLoop };
pub type CraftContext = glutin::WindowedContext<PossiblyCurrent>;

pub struct Window {
    event_loop: EventLoop<()>,
    context: CraftContext,
}

impl Window {
    pub fn create_window() -> Self {
        let el = EventLoop::new();

        let win = WindowBuilder::new()
            .with_inner_size(glutin::dpi::LogicalSize { width: 1024, height: 576 })
            .with_title("gecraftet");

        let ctx = ContextBuilder::new()
            .with_gl(GlRequest::Specific(GlApi::OpenGl, (4, 0)))
            .with_gl_profile(GlProfile::Core)
            .build_windowed(win, &el)
            .map_err(|e| panic!("context creation failed due to {}", e))
            .unwrap();
        
        let ctx = unsafe {
            ctx.make_current()
                .map_err(|(_, e)| panic!("unable to make context current due to {}", e))
                .unwrap()
        };
        
        Self {
            event_loop: el,
            context: ctx,
        }
    }

    pub fn context(&self) -> &Context<PossiblyCurrent> {
        self.context.context()
    }

    pub fn run<F>(self, mut callback: F)
        where F: 'static + FnMut(
            Event<()>,
            &mut ControlFlow,
            &CraftContext
        )
    {
        let Self { event_loop, context } = self;

        event_loop.run(move |event, _, cl| {
            *cl = ControlFlow::Poll;
            callback(event, cl, &context);
        });
    }
}
