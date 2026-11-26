#[test]
fn example_script_builds() {
    let src = include_str!("../../../web/example.rhai");
    let out = gd_rust_wasm::compile(src).expect("example should compile");
    assert!(out.gmd.contains("<k>k4</k>"));
    assert_eq!(out.objects, 8);
}

#[test]
fn bad_script_reports_error() {
    assert!(gd_rust_wasm::compile("add(Block::nope(1, 2));").is_err());
}

#[test]
fn every_trigger_namespace_resolves() {
    let src = r#"
        let g = next_group(); let c = next_color(); let i = next_item(); let b = next_block_id();
        add(ColorTrigger::new(c, 255, 0, 0, 1.0, 0.5, false));
        add(PulseTrigger::new(g, PulseMode::Group, 0, 1, 0));
        add(AreaMoveTrigger::new(g, g, #{length: 300, relative: true}, #{dx: 2}));
        add(AreaStopTrigger::new(1));
        add(MoveEnterTrigger::new(g, TransitionMode::Enter, 1));
        add(SequenceTrigger::new([[g, 100], [g, 200]], SeqMode::Loop));
        add(AdvancedRandomTrigger::new([[g, 1], [g, 3]]));
        add(ItemEditTrigger::new(i, i, i, MathOp::Add, 1, 1, RoundMode::None));
        add(CollisionTrigger::new(b, b, g, false));
        add(EventTrigger::new(Event::YELLOW_ORB, g));
        add(OptionsTrigger::new([[OptionFlag::HideGround, true]]));
        add(ShaderTrigger::new(ShaderType::Sepia, #{strength: 1, duration: 2}));
        add(StaticCameraTrigger::new(g, false, true, 1, Easing::SineOut));
        add(Portal::teleport([5, 2], [20, 9]));
        add(Deco::particle_emitter(1, 1, [[Particle::SPEED, 30]]));
        add(Block::default(1, 1).with_z(ZLayer::T1, 3).set_flags([Flag::NoTouch]).remap([[1, 2]]));
    "#;
    let out = gd_rust_wasm::compile(src).unwrap();
    assert_eq!(out.objects, 16);
}
