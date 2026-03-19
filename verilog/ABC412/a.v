`define I 32'h8000_0000
module Main;
integer n;
integer i,ans,a,b;
initial begin
    $fscanf(`I, "%d", n);
    ans=0;
    for (i=0;i<n;i=i+1) begin
        $fscanf(`I, "%d %d", a, b);
        if (b > a) ans = ans + 1;
    end
    $display("%0d", ans);
    $finish(0);
end
endmodule