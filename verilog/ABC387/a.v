`define I 32'h8000_0000
module Main;
integer a,b;
initial begin
    $fscanf(`I, "%d %d", a,b);
    $display("%0d", (a+b)*(a+b));
    $finish(0);
end
endmodule