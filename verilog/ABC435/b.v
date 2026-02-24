`define I 32'h8000_0000
module Main;
integer n,i,l,r;
integer sum,ans;
reg ok;
integer a [50:0];
initial begin
    $fscanf(`I, "%d", n);
    for (i = 0; i < n; i = i + 1) begin
        $fscanf(`I, "%d", a[i]);
    end
    ans=0;
    for (l = 0; l < n; l = l + 1) begin
        sum = 0;
        for (r = l; r < n; r = r + 1) begin
            sum = sum + a[r];
            ok = 1;
            for (i = l; i <= r; i = i + 1) if(sum % a[i] == 0)ok=0;
            ans = ans + ok;
        end
    end
    $display("%0d", ans);
end
endmodule