`define I 32'h8000_0000
module Main;
integer n,i,j,sum;
integer d[50:0];
initial begin
    $fscanf(`I, "%d", n);
    for (i=0;i<n-1;i=i+1) begin
        $fscanf(`I, "%d", d[i]);
    end
    for (i=0;i<n-1;i=i+1) begin
        sum = 0;
        for (j=i;j<n-1;j=j+1) begin
            sum = sum + d[j];
            $write("%0d ", sum);
        end
        $display("");
    end
    $finish(0);
end
endmodule