`define I 32'h8000_0000
module Main;
integer a,b,c;
initial begin
    $fscanf(`I, "%d %d %d", a,b,c);
    if (a == b && b == c || a + b == c || b + c == a || c + a == b) $display("Yes");
    else $display("No");
    $finish(0);
end
endmodule