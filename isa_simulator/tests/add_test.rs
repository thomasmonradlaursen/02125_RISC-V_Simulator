use mylib::simulator;

    #[test]
    fn test_add() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_add.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-100,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b), "Test of ADD failed");
    }