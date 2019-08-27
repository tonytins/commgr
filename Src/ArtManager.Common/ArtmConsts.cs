/*
 * Copyright (c) Anthony Wilcox and contributors. All rights reserved.
 * Licensed under the MPL 2.0 license. See LICENSE file in the project
 * root for full license information.
 */
using System;
using System.IO;

namespace ArtManager.Common
{

    public class ArtmConsts
    {
        public const string DB_ERR = "Failed to connect to database";
        internal const string DEFUALT_DB_FILE = "artm.db";
        public const string DB_COLUMN = "art";
        public const string APP_NAME = "artm";
        public static string AppDataPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), APP_NAME);
    }

}