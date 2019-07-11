// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using EntryPoint;

namespace ArtManager
{
    class Program
    {
        static void Main(string[] args)
        {
            Cli.Execute<CliCmd>(args);
        }
    }
}
