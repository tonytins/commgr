/*
 * Copyright (c) Anthony Wilcox and contributors. All rights reserved.
 * Licensed under the MPL 2.0 license. See LICENSE file in the project
 * root for full license information.
 */
using System;
using System.Collections.Generic;
using System.Security.Cryptography;
using System.Text;
using System.Diagnostics;
using Newtonsoft.Json;
using Bogus;

namespace ArtManager.Models
{
    public static class ArtUtils
    {
        static readonly JsonSerializerSettings _serializerSettings = new JsonSerializerSettings
        {
            NullValueHandling = NullValueHandling.Ignore,
        };

        /// <summary>
        /// Returns a Json string of the Art class, ignoring the
        /// null values.
        /// </summary>
        /// <param name="art"></param>
        /// <returns></returns>
        public static string AsJson(Art art)
        {
            return JsonConvert.SerializeObject(art, Formatting.Indented, _serializerSettings);
        }

        /// <summary>
        /// Returns a Json string of the Art class as an list,
        /// ignoring the null values.
        /// </summary>
        /// <param name="art"></param>
        /// <returns></returns>
        public static string AsJson(List<Art> art)
        {
            return JsonConvert.SerializeObject(art, Formatting.Indented, _serializerSettings);
        }

        /// <summary>
        /// Prints the Json data to the screen
        /// </summary>
        /// <param name="art"></param>
        public static void WriteJson(Art art)
        {
            if (Debugger.IsAttached)
                Debug.WriteLine(AsJson(art));
            else
                Console.WriteLine(AsJson(art));
        }

        /// <summary>
        /// Prints the Json data to the screen
        /// </summary>
        /// <param name="art"></param>
        public static void WriteJson(List<Art> art)
        {
            if (Debugger.IsAttached)
                Debug.WriteLine(AsJson(art));
            else
                Console.WriteLine(AsJson(art));
        }

    }
}
