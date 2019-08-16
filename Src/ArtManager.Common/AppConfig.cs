// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using System.IO;
using Nett;
using Serilog;

namespace ArtManager.Common
{
    public class AppConfig
    {
        const string CFG_FILE = "artm.toml";

        public string Database { get; set; } = "artm.db";

        public bool Debug { get; set; } = false;

        public static AppConfig GetConfig
        {
            get
            {
                var cfgInstance = new AppConfig();

                try
                {
                    if (!File.Exists(CFG_FILE))
                    {
                        Toml.WriteFile(cfgInstance, CFG_FILE);
                        Log.Information($"{CFG_FILE} not found. Creating new file.");
                    }

                    if (File.Exists(CFG_FILE))
                        return Toml.ReadFile<AppConfig>(CFG_FILE);

                    return cfgInstance;
                }
                catch (IOException err)
                {
                    if (!Debugger.IsAttached)
                        Log.Fatal($"{err.Message}{Environment.NewLine}{err.StackTrace}");
                    else
                        Log.Fatal(err.Message);

                    throw new IOException(err.Message);
                }
                catch (Exception err)
                {
                    if (!Debugger.IsAttached)
                        Log.Fatal($"{err.Message}{Environment.NewLine}{err.StackTrace}");

                    throw new Exception(err.Message);
                }
            }
        }
    }
}
