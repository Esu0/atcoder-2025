`define I 32'h8000_0000
module Main;
integer n;
initial begin
    $fscanf(`I, "%d", n);
    if (400 % n == 0) $display("%0d", 400 / n);
    else $display("-1");
    $finish(0);
end
endmodule