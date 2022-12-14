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
        let expected: [i32; 32] = [0,0,0,0,0,102,-128,79,102,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lbu() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_lbu.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,102,128,79,102,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }
    
    #[test]
    fn lh() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_lh.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,13302,-32640,20303,13302,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lhu() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_lhu.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,13302,32896,20303,13302,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn li() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_li.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,0,0,0,-1,29154821,10,0,0,0,0,0,0,0,-1301186204,-12,-1125213234,-16777216,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lui() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_lui.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,4194304,259366912,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lw() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_lw.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,1338802,-2139062144,1330597711,1338802,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn or() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_or.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-1895759887,-1895759887,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn ori() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_ori.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-1895759887,-1895759887,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sb() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sb.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,102,102,0,102,60,10,0,0,0,0,0,0,0,102,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sh() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sh.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,45,10232,0,52781,50,10,0,0,0,0,0,0,0,10232,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sll() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sll.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,251723520,-267390976,-16777216,16,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn slli() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_slli.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,251723520,-267390976,-16777216,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn slt() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_slt.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn slti() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_slti.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sltiu() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sltiu.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sltu() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sltu.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sra() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sra.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,15794160,-15732721,-61456,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,987135,3855,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn srai() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_srai.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,15794160,-15732721,-61456,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,987135,3855,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn srl() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_srl.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-251723536,252702735,987120,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn srli() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_srli.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-251723536,252702735,987120,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }


    #[test]
    fn sub() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sub.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,-100,-125,-160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sw() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_sw.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,45,-50,116,980733485,50,10,0,0,0,0,0,0,0,10232,10232,0,0,0,0,0,0,0,0,58,-12755,14964,29902];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn xor() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_xor.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,251723760,1807,251721983,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn xori() {
        let reg = simulator::get_computation(&String::from("instruction_tests/test_xori.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,251723760,251721983,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }