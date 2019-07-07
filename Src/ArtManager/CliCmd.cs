// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using EntryPoint;

namespace ArtManager
{
    class CliCmd : BaseCliCommands
    {
        [DefaultCommand]
        public void Primary(string[] args)
        {
            var cli = Cli.Parse<CliArgs>(args);

            if (cli.Debug)
            {
                Console.WriteLine("It works!");
            }
        }

        [Command("raffle")]
        public void Raffle(string[] args)
        {
            var cli = Cli.Parse<CliArgs>(args);
        }
    }
}
