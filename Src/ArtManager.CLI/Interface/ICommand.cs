/*
 * Copyright (c) Anthony Wilcox and contributors. All rights reserved.
 * Licensed under the MPL 2.0 license. See LICENSE file in the project
 * root for full license information.
 */
namespace ArtManager.CLI.Interface
{
    interface ICommand
    {
        int RunCommand(IBaseArgs cli);
    }
}
