// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System.IO;
using Bodkin.Serilog;
using Nett;
using Serilog;

namespace ArtManager.Common
{
    public class AppConfig
    {
        const string CFG_FILE = "artm.toml";

        public string Database { get; set; } = Path.Combine(ArtmConsts.AppDataPath, ArtmConsts.DEFUALT_DB_FILE);

        public bool Debug { get; set; } = false;

        public static AppConfig GetConfig
        {
            get
            {
                var cfgInstance = new AppConfig();
                var cfgPath = Path.Combine(ArtmConsts.AppDataPath, CFG_FILE);

                try
                {
                    if (!File.Exists(cfgPath))
                    {
                        Toml.WriteFile(cfgInstance, cfgPath);
                        Log.Information($"{cfgPath} not found. Creating new file.");
                    }

                    if (File.Exists(cfgPath))
                        return Toml.ReadFile<AppConfig>(cfgPath);

                    return cfgInstance;
                }
                catch (IOException err)
                {
                    SerilogHelper.LogException(err);

                    throw new IOException(err.Message);
                }
            }
        }
    }
}