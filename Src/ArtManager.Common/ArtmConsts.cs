using System;
using System.IO;

namespace ArtManager.Common
{

    public class ArtmConsts
    {
        public const string DB_ERR = "Failed to connect to database";
        internal const string DEFUALT_DB_FILE = "artm.db";
        public const string DB_COLUMN = "art";
        public static string AppDataPath = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), DEFUALT_DB_FILE);
    }

}