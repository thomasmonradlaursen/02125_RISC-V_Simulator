use mylib::simulator_engine;

const STEPWISE: bool = false;
const HAZARD: bool = true;

    #[test]
    fn add() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_add.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-100,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn addi() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_addi.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,0,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn and() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_and.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,251723761,1793,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn andi() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_andi.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,251723761,1793,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn auipc() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_auipc.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,4194304,4194308,1883242504,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn beq() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_beq.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bge() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_bge.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bgeu() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_bgeu.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn blt() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_blt.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bltu() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_bltu.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn bne() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_bne.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    /*#[test]
    fn ecall() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_ecall.bin"));
        let expected: [i32; 32] = [0,0,0,0,0,0,0,0,1,1,10,10,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }*/

    #[test]
    fn jal() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_jal.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,4,16,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn jalr() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_jalr.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,4,20,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lb() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_lb.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,102,-128,79,102,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lbu() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_lbu.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,102,128,79,102,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }
    
    #[test]
    fn lh() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_lh.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,13302,-32640,20303,13302,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lhu() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_lhu.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,13302,32896,20303,13302,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn li() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_li.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,0,0,0,-1,29154821,10,0,0,0,0,0,0,0,-1301186204,-12,-1125213234,-16777216,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lui() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_lui.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,4194304,259366912,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn lw() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_lw.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,1338802,-2139062144,1330597711,1338802,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn or() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_or.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-1895759887,-1895759887,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn ori() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_ori.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-1895759887,-1895759887,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sb() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sb.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,102,102,0,102,60,10,0,0,0,0,0,0,0,102,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sh() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sh.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,45,10232,0,52781,50,10,0,0,0,0,0,0,0,10232,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sll() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sll.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,251723520,-267390976,-16777216,16,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn slli() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_slli.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,251723520,-267390976,-16777216,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn slt() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_slt.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn slti() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_slti.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sltiu() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sltiu.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sltu() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sltu.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sra() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sra.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,15794160,-15732721,-61456,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,987135,3855,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn srai() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_srai.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,15794160,-15732721,-61456,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,987135,3855,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn srl() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_srl.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-251723536,252702735,987120,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn srli() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_srli.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-251723536,252702735,987120,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }


    #[test]
    fn sub() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sub.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,-100,-125,-160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-60,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn sw() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_sw.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,45,-50,116,980733485,50,10,0,0,0,0,0,0,0,10232,10232,0,0,0,0,0,0,0,0,58,-12755,14964,29902];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn xor() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_xor.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,251723760,1807,251721983,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn xori() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_xori.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,0,0,0,0,251723760,251721983,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }

    #[test]
    fn load_use() {
        let reg = simulator_engine::run_simulation(&String::from("instruction_tests/test_load_use.bin"), STEPWISE, HAZARD);
        let expected: [i32; 32] = [0,1,2,1,3,-1,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        assert!(reg.iter().zip(expected.iter()).all(|(a,b)| a == b));
    }