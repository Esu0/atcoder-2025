`define I 32'h8000_0000
module Main;
integer n,i,a,sum;
initial begin
    $fscanf(`I, "%d", n);
    sum = 0;
    for (i=0;i<n;i=i+1) begin
        $fscanf(`I, "%d", a);
        if (i % 2 == 0) sum = sum + a;
    end
    $display("%0d", sum);
    $finish(0);
end
endmodule