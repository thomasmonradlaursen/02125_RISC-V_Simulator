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

    #[test]
    fn andi() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_andi.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,251723761,1793,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn auipc() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_auipc.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,4194304,4194308,1883242504,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn beq() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_beq.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bge() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_bge.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bgeu() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_bgeu.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn blt() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_blt.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bltu() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_bltu.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bne() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_bne.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    /*#[test]
    fn ecall() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_ecall.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,0,0,0,1,1,10,10,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }*/

    #[test]
    fn jal() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_jal.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,4,16,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn jalr() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_jalr.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,4,20,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lb() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_lb.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,4,20,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }
