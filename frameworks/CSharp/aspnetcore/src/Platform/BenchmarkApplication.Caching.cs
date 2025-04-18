﻿// Copyright (c) .NET Foundation. All rights reserved.
// Licensed under the Apache License, Version 2.0. See License.txt in the project root for license information.

using System.IO.Pipelines;
using System.Threading.Tasks;

namespace PlatformBenchmarks;

public sealed partial class BenchmarkApplication
{
    private static async Task Caching(PipeWriter pipeWriter, int count)
    {
        OutputMultipleQueries(pipeWriter, await RawDb.LoadCachedQueries(count), SerializerContext.CachedWorldArray);
    }
}
