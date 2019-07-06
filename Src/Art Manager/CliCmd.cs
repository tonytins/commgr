// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using EntryPoint;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json.Serialization;

namespace Art_Manager
{
    class CliCmd : BaseCliCommands
    {
        [Command("raffle")]
        public void Raffle(string[] args)
        {
            var cli = Cli.Parse<CliArgs>(args);
        }
    }
}
