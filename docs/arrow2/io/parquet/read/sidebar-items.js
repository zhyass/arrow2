initSidebarItems({"enum":[["DataPageHeader",""],["LogicalType",""],["ParquetError","Errors generated by this crate"],["ParquetTimeUnit",""],["ParquetType","Representation of a Parquet type. Used to describe primitive leaf fields and structs, including top-level schema. Note that the top-level schema type is represented using `GroupType` whose repetition is `None`."],["PhysicalType",""],["PrimitiveConvertedType",""],["State","State of [`MutStreamingIterator`]."]],"fn":[["_get_page_iterator","Returns a new [`PageIterator`] by seeking `reader` to the begining of `column_chunk`."],["_get_page_stream","Returns a stream of compressed data pages"],["_read_metadata","Reads a file’s metadata."],["_read_metadata_async",""],["column_iter_to_array","Returns an [`Array`] built from an iterator of column chunks. It also returns the two buffers used to decompress and deserialize pages (to be re-used)."],["decompress","Decompresses the page, using `buffer` for decompression. If `page.buffer.len() == 0`, there was no decompression and the buffer was moved. Else, decompression took place."],["get_column_iterator","Returns a [`ColumnIterator`] of column chunks corresponding to `field`. Contrarily to [`get_page_iterator`] that returns a single iterator of pages, this iterator returns multiple iterators, one per physical column of the `field`. For primitive fields (e.g. `i64`), [`ColumnIterator`] yields exactly one column. For complex fields, it yields multiple columns."],["get_page_iterator","Creates a new iterator of compressed pages."],["get_page_stream","Creates a new iterator of compressed pages."],["int96_to_i64_ns",""],["page_stream_to_array","Converts an async stream of [`DataPage`] into a single [`Array`]."],["read_metadata","Reads parquets’ metadata syncronously."],["read_metadata_async","Reads parquets’ metadata asynchronously."]],"mod":[["schema","APIs to handle Parquet <-> Arrow schemas."],["statistics","APIs exposing `parquet2`’s statistics as arrow’s statistics."]],"struct":[["BasicDecompressor","A [`FallibleStreamingIterator`] that decompresses [`CompressedDataPage`] into [`DataPage`]."],["ColumnChunkMetaData","Metadata for a column chunk."],["ColumnDescriptor","A descriptor for leaf-level primitive columns. This encapsulates information such as definition and repetition levels and is used to re-assemble nested data."],["CompressedDataPage","A [`CompressedDataPage`] is compressed, encoded representation of a Parquet data page. It holds actual data and thus cloning it is expensive."],["DataPage","A [`DataPage`] is an uncompressed, encoded representation of a Parquet data page. It holds actual data and thus cloning it is expensive."],["Decompressor","Decompressor that allows re-using the page buffer of [`PageIterator`]."],["FileMetaData","Metadata for a Parquet file."],["PageIterator","A page iterator iterates over row group’s pages. In parquet, pages are guaranteed to be contiguously arranged in memory and therefore must be read in sequence."],["ReadColumnIterator","A [`MutStreamingIterator`] of pre-read column chunks"],["RecordReader","Single threaded iterator of a paquet file."],["RowGroupMetaData","Metadata for a row group."],["TimestampType","Timestamp logical type annotation"]],"trait":[["ColumnChunkIter","Trait describing a [`MutStreamingIterator`] of column chunks."],["FallibleStreamingIterator","A fallible, streaming iterator."],["MutStreamingIterator",""]],"type":[["PageFilter","Type declaration for a page filter"]]});