// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using EntryPoint;

namespace ArtManager
{
    class CliCmd : BaseCliCommands
    {
        [DefaultCommand]
        [Command("req")]
        public void Request(string[] args)
        {
            var cli = Cli.Parse<ReqArgs>(args);
            Order.Request(cli.Name, cli.Customer, cli.Description, cli.Debug);
        }

        [Command("com")]
        public void Commission(string[] args)
        {
            var cli = Cli.Parse<ComArgs>(args);
            Order.Commission(cli.Name, cli.Customer, cli.Description, cli.Price, cli.Payment, cli.Debug);
        }
    }
}