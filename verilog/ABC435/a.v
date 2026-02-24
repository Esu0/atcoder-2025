`define I 32'h8000_0000
module Main;
integer n;
initial begin
    $fscanf(`I, "%d", n);
    $display("%0d", n*(n+1)/2);
end
endmodule