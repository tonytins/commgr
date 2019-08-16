// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the MIT license. See LICENSE file in the project
// root for full license information.
using System;
using System.Runtime.InteropServices;

namespace Bodkin
{
    // Extended from https://mariusschulz.com/blog/detecting-the-operating-system-in-net-core
    public static class PlatformDetect
    {
        /// <summary>
        /// Detects if Windows is present and returns the Unix
        /// platform ID if it isn't.
        /// </summary>
        public static PlatformID IsPlatformID
        {
            get
            {
                if (IsWindows)
                    return PlatformID.Win32NT;
                else
                    return PlatformID.Unix;
            }
        }

        /// <summary>
        /// Similar to IsPlatFormID but takes macOS into account using
        /// Bodkin's SystemID enum.
        /// </summary>
        public static SystemID IsSystemID
        {
            get
            {
                if (IsWindows)
                    return SystemID.WinNT;
                else if (IsMacOS)
                    return SystemID.MacOS;
                else if (IsLinux)
                    return SystemID.Linux;
                else
                    return SystemID.Unknown;
            }
        }

        public static bool IsWindows
        {
            get
            {
                return RuntimeInformation.IsOSPlatform(OSPlatform.Windows);
            }
        }

        public static bool IsMacOS
        {
            get
            {
                return RuntimeInformation.IsOSPlatform(OSPlatform.OSX);
            }
        }

        public static bool IsLinux
        {
            get
            {
                return RuntimeInformation.IsOSPlatform(OSPlatform.Linux);
            }
        }
    }
}
