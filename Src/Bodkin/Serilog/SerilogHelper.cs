// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
using System;
using System.Diagnostics;
using Serilog;

namespace Bodkin.Serilog {
    public class SerilogHelper {
        /// <summary>
        /// Logs the exception error message. The stack trace is included if the VS
        /// debugger isn't attached, or if the stackTrace is checked.
        /// </summary>
        /// <param name="err">Any exception</param>
        /// <param name="stackTrace">false, by default</param>
        public static void LogException (Exception err, bool stackTrace = false) {
            if (!Debugger.IsAttached || stackTrace)
                Log.Debug ($"{err.Message}{Environment.NewLine}{err.StackTrace}");
            else
                Log.Fatal (err.Message);
        }

        /// <summary>
        /// Logs a custom message with the exception. The stack trace is included if the VS
        /// debugger isn't attached, or if the stackTrace is checked.
        /// </summary>
        /// <param name="message">Custom message</param>
        /// <param name="err">Any exception</param>
        /// <param name="stackTrace">false, by default</param>
        public static void LogException (string message, Exception err, bool stackTrace = false) {
            if (!Debugger.IsAttached || stackTrace)
                Log.Debug ($"{message}{Environment.NewLine}{err.StackTrace}");
            else
                Log.Fatal (message);
        }
    }
}