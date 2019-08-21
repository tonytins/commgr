namespace ArtManager.CLI.Interface
{
    interface ICommand
    {
        int RunCommand(IBaseArgs cli);
    }
}
