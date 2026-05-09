import iq_dump as iq

def main():
    iq.init_logger()
    dut = iq.PyDut("192.168.1.1:9600")

    dut.ate_init()
    dut.shut_down_band("HB")
    
    config_rf_dc(dut)
    # -------------------------------------------
    dut.open_rx("LB", 2422)
    # dut.run_test("LB", "Fem", list(range(0, 2)))
    # dut.run_test("LB", "Lna", list(range(1, 8)))
    dut.run_test("LB", "Vga", list(range(0, 32)))
    iq.set_phynum("lb", 0)
    dut.close_rx("LB")

    # -------------------------------------------
    dut.open_rx("LB", 2442)
    dut.run_test("LB", "Vga", list(range(0, 32)))
    dut.close_rx("LB")

    # -------------------------------------------
    dut.open_rx("LB", 2462)
    dut.run_test("LB", "Vga", list(range(0, 32)))
    dut.close_rx("LB")


    dut.shut_up_band("HB")
    dut.shut_down_band("LB")

    config_rf_dc(dut)

    #--------------------------------------------
    dut.open_rx("HB", 5210)
    dut.run_test("HB", "Vga", list(range(0, 32)))
    dut.close_rx("HB")

    #--------------------------------------------
    dut.open_rx("HB", 5290)
    dut.run_test("HB", "Vga", list(range(0, 32)))
    dut.close_rx("HB")

    #--------------------------------------------
    dut.open_rx("HB", 5530)
    dut.run_test("HB", "Vga", list(range(0, 32)))
    dut.close_rx("HB")

    #--------------------------------------------
    dut.open_rx("HB", 5610)
    dut.run_test("HB", "Vga", list(range(0, 32)))
    dut.close_rx("HB")

    #--------------------------------------------
    dut.open_rx("HB", 5690)
    dut.run_test("HB", "Vga", list(range(0, 32)))
    dut.close_rx("HB")

    #--------------------------------------------
    dut.open_rx("HB", 5775)
    dut.run_test("HB", "Vga", list(range(0, 32)))
    dut.close_rx("HB")

    dut.parse()


    print("Over!")


def config_rf_dc(dut):
    dut.devmem(0x05000000, 0x18544)
    dut.devmem(0x05000000, 0x18545)
    dut.devmem(0x05000000, 0x18548)
    dut.devmem(0x05000000, 0x18549)
    # 2G0
    dut.devmem(0x05000000, 0x7e854d)
    dut.devmem(0x05000000, 0x1854e)
    dut.devmem(0x05000000, 0x7f854f)
    dut.devmem(0x05000000, 0x18550)
    # 5G0
    dut.devmem(0x05000000, 0x838555)
    dut.devmem(0x05000000, 0x8556)
    dut.devmem(0x05000000, 0x7e8557)
    dut.devmem(0x05000000, 0x18558)
    dut.devmem(0x05000000, 0x18546)
    dut.devmem(0x05000000, 0x18547)
    dut.devmem(0x05000000, 0x1854a)
    dut.devmem(0x05000000, 0x1854b)
    # 2G1
    dut.devmem(0x05000000, 0x7f8551)
    dut.devmem(0x05000000, 0x18552)
    dut.devmem(0x05000000, 0x7f8553)
    dut.devmem(0x05000000, 0x18554)
    # 5G1
    dut.devmem(0x05000000, 0x7e8559)
    dut.devmem(0x05000000, 0x1855a)
    dut.devmem(0x05000000, 0x80855b)
    dut.devmem(0x05000000, 0x855c)

if __name__ == "__main__":
    main()