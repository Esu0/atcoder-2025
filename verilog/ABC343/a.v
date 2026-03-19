`define I 32'h8000_0000
module Main;
integer a,b;
initial begin
    $fscanf(`I, "%d %d", a,b);
    if (a+b==0) $display("1");
    else $display("0");
    $finish(0);
end
endmodule