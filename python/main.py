import iq_dump as iq

def main():
    iq.init_logger()
    dut = iq.PyDut("192.168.1.1:9600")

    dut.ate_init()
    dut.shut_down_band("HB")
    dut.open_rx("LB")

    dut.run_test("LB", "Fem", list(range(0, 2)))
    dut.run_test("LB", "Lna", list(range(1, 8)))
    dut.run_test("LB", "Vga", list(range(1, 21)))

    dut.close_rx("LB")
    dut.shut_up_band("HB")
    dut.shut_down_band("LB")
    dut.open_rx("HB")

    dut.run_test("HB", "Fem", list(range(0, 2)))
    dut.run_test("HB", "Lna", list(range(1, 8)))
    dut.run_test("LB", "Vga", list(range(1, 21)))

    dut.parse()


    print("Over!")


if __name__ == "__main__":
    main()