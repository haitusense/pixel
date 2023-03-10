#r "nuget: Parquet.Net, 4.5.4"
// パーケット
using Parquet;
using Parquet.Data;
using System;
using System.Linq;

using(Stream fs = System.IO.File.OpenWrite("temp.parquet")) {
    using(ParquetWriter writer = await ParquetWriter.CreateAsync(schema, fs)) {
        using(ParquetRowGroupWriter groupWriter = writer.CreateRowGroup()) {
            
            await groupWriter.WriteColumnAsync(column1);
            await groupWriter.WriteColumnAsync(column2);
            await groupWriter.WriteColumnAsync(column3);
            
        }
    }
}