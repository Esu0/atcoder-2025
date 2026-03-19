`define I 32'h8000_0000
module Main;
integer n;
initial begin
    $fscanf(`I, "%d", n);
    $write("1");
    repeat(n) $write("01");
    $display();
    $finish(0);
end
endmodule