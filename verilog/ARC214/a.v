`define I 32'h8000_0000
module Main;
integer N,i,j,c;
integer num [199:0];

initial begin
    $fscanf(`I, "%d", N);
    for (i=0;i<2*N;i=i+1)begin
        num[i] = 0;
    end
    $fgetc(`I);
    for (i=0;i<N;i=i+1)begin
        for (j=0;j<N;j=j+1)begin
            c = $fgetc(`I);
            // $display("debug: %c", c);
            if (c != 63) begin
                if (num[i+j] == 0) num[i+j]=c;
                if (num[i+j]!=c) begin
                    $display("-1");
                    $finish(0);
                end
            end
        end
        $fgetc(`I);
    end
    for (i=0;i<N;i=i+1)begin
        for (j=0;j<N;j=j+1)begin
            if (num[i+j]==0)num[i+j] = 48;
            $write("%c", num[i+j]);
        end
        $write("\n");
    end
    $finish(0);
end
endmodule
