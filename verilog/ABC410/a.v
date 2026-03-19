`define I 32'h8000_0000
module Main;
integer n,i,k,ans;
integer a[100:0];
initial begin
    $fscanf(`I, "%d", n);
    for (i=0;i<n;i=i+1) begin
        $fscanf(`I, "%d", a[i]);
    end
    $fscanf(`I, "%d", k);
    ans=0;
    for (i=0;i<n;i=i+1) begin
        if (k <= a[i]) ans = ans + 1;
    end
    $display("%0d", ans);
    $finish(0);
end
endmodule