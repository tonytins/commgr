namespace ArtManager.CLI.Interface
{
    interface ICommand
    {
        int RunCommand(IBaseArgs cli);

        int RunCommand(IYchArgs cli);

        int RunCommand(ICustArgs cli);

        int RunCommand(IPayArgs cli);
    }
}
