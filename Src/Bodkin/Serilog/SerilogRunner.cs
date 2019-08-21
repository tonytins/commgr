// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
using System;
using System.IO;
using Serilog;
using Serilog.Events;

namespace Bodkin.Serilog
{
    public static class SerilogRunner
    {
        /// <summary>
        /// Initializes a Serilog configuration to output to the debugger and file to the application's directory.
        /// WARNING: This not work as intended for bundled single file executables.
        /// </summary>
        /// <param name="dir">"log"</param>
        /// <param name="interval">RollingInterval.Month</param>
        /// <param name="minLogLevel">LogEventLevel.Information</param>
        /// <param name="minDebugLogLevel">LogEventLevel.Verbose</param>
        public static void InitLogToFile(string prefix = "log",
            RollingInterval interval = RollingInterval.Month,
            LogEventLevel minLogLevel = LogEventLevel.Information, LogEventLevel minDebugLogLevel = LogEventLevel.Verbose)
        {
            var logFile = Path.GetFullPath(Path.Combine(AppDomain.CurrentDomain.BaseDirectory, $"{prefix}-.log"));
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Debug(restrictedToMinimumLevel: minDebugLogLevel)
                .WriteTo.File(logFile, rollingInterval: interval, restrictedToMinimumLevel: minLogLevel)
                .CreateLogger();
        }

        /// <summary>
        /// Initializes a Serilog configuration to output to the debugger and file with a custom directory.
        /// </summary>
        /// <param name="customDir"></param>
        /// <param name="dir">"log"</param>
        /// <param name="interval">RollingInterval.Month</param>
        /// <param name="minLogLevel">LogEventLevel.Information</param>
        /// <param name="minDebugLogLevel">LogEventLevel.Verbose</param>
        public static void InitLogToDirectory(string customDir, string prefix = "log",
            RollingInterval interval = RollingInterval.Month,
            LogEventLevel minLogLevel = LogEventLevel.Information, LogEventLevel minDebugLogLevel = LogEventLevel.Verbose)
        {
            var logFile = Path.GetFullPath(Path.Combine(customDir, $"{prefix}-.log"));
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Debug(restrictedToMinimumLevel: minDebugLogLevel)
                .WriteTo.File(logFile, rollingInterval: interval, restrictedToMinimumLevel: minLogLevel)
                .CreateLogger();
        }

        /// <summary>
        /// Initializes a Serilog configuration to output to the debugger and terminal.
        /// </summary>
        /// <param name="minLogLevel">LogEventLevel.Information</param>
        /// <param name="minDebugLogLevel">LogEventLevel.Verbose</param>
        public static void InitLogToConsole(LogEventLevel minLogLevel = LogEventLevel.Information,
            LogEventLevel minDebugLogLevel = LogEventLevel.Verbose)
        {
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Console(restrictedToMinimumLevel: minLogLevel)
                .WriteTo.Debug(restrictedToMinimumLevel: minDebugLogLevel)
                .CreateLogger();
        }

        /// <summary>
        /// Initializes a Serilog configuration to only output to the debugger.
        /// </summary>
        /// <param name="minEventLevel">LogEventLevel.Verbose</param>
        public static void InitLogToDebug(LogEventLevel minEventLevel = LogEventLevel.Verbose)
        {
            Log.Logger = new LoggerConfiguration()
                .MinimumLevel.Debug()
                .WriteTo.Debug(restrictedToMinimumLevel: minEventLevel)
                .CreateLogger();
        }
    }
}