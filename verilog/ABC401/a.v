`define I 32'h8000_0000
module Main;
integer n;
initial begin
    $fscanf(`I, "%d", n);
    if (200 <= n && n <= 299) $display("Success");
    else $display("Failure");
    $finish(0);
end
endmodule