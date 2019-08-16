// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
using Serilog;
using Serilog.Events;

namespace ArtManager.Common
{
    public class LogRunner
    {
        /// <summary>
        /// Initializes a Serilog configuration with file output, minimum logging level set to debug and
        /// printing to console enabled. File settings may changed in the parameters.
        /// </summary>
        /// <param name="logFile">"log-.txt"</param>
        /// <param name="interval">RollingInterval.Month</param>
        /// <param name="minEventLevel">LogEventLevel.Information</param>
        public static void InitLogToFile(string logFile = "log-.txt",
            RollingInterval interval = RollingInterval.Month,
            LogEventLevel minEventLevel = LogEventLevel.Information)
        {
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Console()
                .WriteTo.File(logFile, rollingInterval: interval, restrictedToMinimumLevel: minEventLevel)
                .CreateLogger();
        }

        /// <summary>
        /// Initializes a Serilog configuration with minimum logging level set to debug and
        /// printing to console enabled.
        /// </summary>
        public static void InitLogToConsole()
        {
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Console()
                .CreateLogger();
        }

        public static void InitLogToDebug()
        {
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Debug()
                .CreateLogger();
        }
    }
}
