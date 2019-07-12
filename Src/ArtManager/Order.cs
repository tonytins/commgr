// Copyright (c) Anthony Wilcox and contributors. All rights reserved.
// Licensed under the GNU GPL v3 license. See LICENSE file in the project
// root for full license information.
using System;
using System.IO;
using System.Text.Json.Serialization;
using System.Text;
using System.Threading.Tasks;

namespace ArtManager
{
    class Order
    {
        Art Art { get; set; }

        public Order(Art order)
        {
            Art = order;
        }

        public string JsonString
        {
            get
            {
                var op = new JsonSerializerOptions()
                {
                    IgnoreNullValues = true,
                    WriteIndented = true,
                };

                return JsonSerializer.ToString(Art, op);
            }
        }

        public async Task JsonFileAsync(string path)
        {
            try
            {
                if (!File.Exists(path))
                {
                    var encodedTxt = Encoding.Unicode.GetBytes(JsonString);

                    using var fstream = new FileStream(path, FileMode.Create, FileAccess.Write, FileShare.None, 4096, true);

                    await fstream.WriteAsync(encodedTxt, 0, encodedTxt.Length);
                }
            }
            catch (IOException ex)
            {
                throw new IOException(ex.Message);
            }
        }
    }
}
