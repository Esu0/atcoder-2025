`define I 32'h8000_0000
module Main;
integer a,b;
initial begin
    $fscanf(`I, "%d %d", a,b);
    if (a == b) $display("1");
    else if (a%2 == b%2) $display("3");
    else $display("2");
    $finish(0);
end
endmodule