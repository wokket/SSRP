# SSRP (SQL Server Resolution Protocol) - Implemented in rust

MS SQL Server supports running multiple instances on the same machine, using distinct names (Named Instances).  Multiple separate instances can't all share the default SQL Server port (`1433`), so by default they start on a random high port, and the SQL client requests the actual runtime connection info from the `Sql Server Browser Service`.

This set of Requests/Responses occurs over UDP using the [Sql Server Resolution Protocol](https://docs.microsoft.com/en-us/openspecs/windows_protocols/mc-sqlr/1ea6e25f-bff9-4364-ba21-5dc449a601b7).

This library aims to provide a rust implementation for this protocol, which could be used prior to connecting to SQL server using [`tiberius`](https://github.com/steffengy/tiberius).

