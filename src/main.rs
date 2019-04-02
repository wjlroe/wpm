use gfx::traits::FactoryExt;
use gfx::{self, *};
use gfx_glyph::*;
use glutin::dpi::*;
use glutin::Api::OpenGl;
use glutin::*;
use std::error::Error;
use std::ops::Add;
use std::time::{Duration, Instant, SystemTime};

type ColorFormat = format::Rgba8;
type DepthFormat = format::Depth;

const QUAD: [Vertex; 4] = [
    Vertex { pos: [-1.0, 1.0] },
    Vertex { pos: [-1.0, -1.0] },
    Vertex { pos: [1.0, -1.0] },
    Vertex { pos: [1.0, 1.0] },
];
const QUAD_INDICES: [u16; 6] = [0u16, 1, 2, 2, 3, 0];

gfx_defines! {
  vertex Vertex {
    pos: [f32; 2] = "a_Pos",
  }

  constant Locals {
    transform: [[f32; 4]; 4] = "u_Transform",
    color: [f32; 3] = "u_Color",
  }

  pipeline pipe {
    vbuf: VertexBuffer<Vertex> = (),
    locals: ConstantBuffer<Locals> = "Locals",
    out_color: BlendTarget<ColorFormat> = ("Target0", state::ColorMask::all(), preset::blend::ALPHA),
    out_depth: DepthTarget<DepthFormat> = preset::depth::LESS_EQUAL_WRITE,
  }
}

#[derive(Copy, Clone, Default, PartialEq)]
struct TypingResult {
    correct_words: i32,
    incorrect_words: i32,
    backspaces: i32,
    wpm: i32,
}

fn calc_wpm(
    wordlist: &Vec<&str>,
    input: &Vec<&str>,
    duration: Duration,
    num_backspaces: i32,
) -> TypingResult {
    let mut typing_result = TypingResult::default();
    typing_result.backspaces = num_backspaces;
    for (entered_word, required_word) in input.iter().zip(wordlist.iter()) {
        if entered_word == required_word {
            typing_result.correct_words += 1;
        } else {
            typing_result.incorrect_words += 1;
        }
    }
    typing_result.wpm =
        (typing_result.correct_words as f64 / (duration.as_secs() as f64 / 60.0)).floor() as i32;
    typing_result
}

#[test]
fn test_calc_wpm() {
    let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
    let words = wordlist.split('|').collect::<Vec<_>>();
    let input = "also sentence stop she men see been from we follow but mother too form this went to then show have only now around help family old write grow also over together city end quite4 with might eat four where hard their take year see place learve too too is other near around saw did into question work between your face without tree as girl if enough stop still put on side there hear large more be there took some into off down so is";
    let input_words = input.split(' ').collect::<Vec<_>>();
    let start_time = SystemTime::UNIX_EPOCH.add(Duration::from_secs(1554148227));
    let end_time = SystemTime::UNIX_EPOCH.add(Duration::from_secs(1554148287));
    let duration = end_time.duration_since(start_time).unwrap();
    let num_backspaces = 11;
    assert_eq!(
        82,
        calc_wpm(&words, &input_words, duration, num_backspaces).wpm
    );

    assert_eq!(
        164,
        calc_wpm(
            &words,
            &input_words,
            Duration::from_secs(30),
            num_backspaces
        )
        .wpm
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    // open a window, process key input
    // attach a gfx context to it...
    let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
    let words = wordlist.split('|').collect::<Vec<_>>();
    let mut typed_in_words: Vec<String> = vec![];

    let background = [0.22, 0.55, 0.3, 1.0];

    let mut event_loop = EventsLoop::new();
    let logical_size = LogicalSize::new(1024.0, 768.0);
    let mut monitor = event_loop.get_primary_monitor();
    let dpi = monitor.get_hidpi_factor() as f32;

    let window_builder = WindowBuilder::new()
        .with_title("wpm")
        .with_dimensions(logical_size);
    let context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl, (4, 3)))
        .with_gl_profile(GlProfile::Core)
        .with_vsync(true);
    let (gfx_window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &event_loop)
            .expect("init gfx_window_glutin should work!");

    let quad_pso = factory
        .create_pipeline_simple(
            include_bytes!("shaders/quad_150_core.vert"),
            include_bytes!("shaders/quad_150_core.frag"),
            pipe::new(),
        )
        .expect("quad pso construction to work");
    let (quad_vbuf, quad_slice) =
        factory.create_vertex_buffer_with_slice(&QUAD, &QUAD_INDICES as &[u16]);
    let mut data = pipe::Data {
        vbuf: quad_vbuf,
        locals: factory.create_constant_buffer(2),
        out_color: main_color,
        out_depth: main_depth,
    };

    let fonts: Vec<&[u8]> = vec![include_bytes!("iosevka-regular.ttf")];

    let mut glyph_brush = GlyphBrushBuilder::using_fonts_bytes(fonts)
        .initial_cache_size((512, 512))
        .depth_test(gfx::preset::depth::LESS_EQUAL_WRITE)
        .build(factory.clone());

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut running = true;

    while running {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::ReceivedCharacter(typed_char) => {
                    println!("You typed: {}", typed_char);
                }
                WindowEvent::CloseRequested | WindowEvent::Destroyed => running = false,
                _ => {}
            },
            _ => {}
        });

        encoder.clear(&data.out_color, background);
        encoder.clear_depth(&data.out_depth, 1.0);

        #[cfg(nope)]
        {
            // draw some sort of quad thingy
            let locals = Locals {
                color,
                transform: transform.into(),
            };
            encoder.update_constant_buffer(&data.locals, &locals);
            encoder.draw(&quad_slice, &quad_pso, &data);
        }

        // end of frame stuff now
        encoder.flush(&mut device);
        gfx_window.swap_buffers()?;
        device.cleanup();
    }

    Ok(())
}
