`define I 32'h8000_0000
module Main;
integer n,i;
integer a[100:0];
integer x;
initial begin
    $fscanf(`I, "%d", n);
    for (i = 0; i < n; i = i + 1) begin
        $fscanf(`I, "%d", a[i]);
    end
    $fscanf(`I, "%d", x);
    for (i = 0; i < n; i = i + 1) begin
        if (a[i] == x) begin
            $display("Yes");
            $finish(0);
        end
    end
    $display("No");
    $finish(0);
end
endmodule