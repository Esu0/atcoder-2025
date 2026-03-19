`define I 32'h8000_0000
module Main;
integer n,m,i;
integer a;
initial begin
    $fscanf(`I, "%d %d", n, m);
    for (i = 0; i < n; i = i + 1) begin
        $fscanf(`I, "%d", a);
        m = m - a;
    end
    if (m < 0) $display("No");
    else $display("Yes");
    $finish(0);
end
endmodule