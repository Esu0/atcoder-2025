`define I 32'h8000_0000
module Main;
integer n;
initial begin
    $fscanf(`I, "%d", n);
    repeat((n-1)/2) $write("-");
    if (n % 2 == 0) $write("==");
    else $write("=");
    repeat((n-1)/2) $write("-");
    $display();
    $finish(0);
end
endmodule