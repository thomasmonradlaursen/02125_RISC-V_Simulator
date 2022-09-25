use mylib::simulator;

    #[test]
    fn add() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_add.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-100,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn addi() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_addi.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,0,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn and() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_and.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,251723761,1793,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }