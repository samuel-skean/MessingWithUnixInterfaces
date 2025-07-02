// Motivating Questions:
// 1. What are the conditions under which a signal's disposition gets reset by an `execve`?
// 2. What constitutes the "disposition" that is reset? The whole `struct sigaction`, or just the `sa_handler`/`sa_sigaction` field?
//
// Tentative answers:
// 1. The disposition is reset only when a handler is established for that signal. This is supported by [`man 7 signal`](https://man7.org/linux/man-pages/man7/signal.7.html#:~:text=During%20an%20execve(2)%2C%20the%20dispositions%20of%20handled%0A%20%20%20%20%20%20%20signals%20are%20reset%20to%20the%20default%3B%20the%20dispositions%20of%20ignored%0A%20%20%20%20%20%20%20signals%20are%20left%20unchanged.) and [`man 2 sigaction`](https://man7.org/linux/man-pages/man2/sigaction.2.html#:~:text=During%20an%20execve(2)%2C%20the%20dispositions%20of%20handled%0A%20%20%20%20%20%20%20signals%20are%20reset%20to%20the%20default%3B%20the%20dispositions%20of%20ignored%0A%20%20%20%20%20%20%20signals%20are%20left%20unchanged.) pretty conclusively:
// > During an execve(2), the dispositions of handled
// > signals are reset to the default; the dispositions of ignored
// > signals are left unchanged.
// That doesn't cover the case where the signal is set to SIG_DFL but somehow otherwise could be affected by resetting - whether such a signal could be affected by resetting is covered by my second question.
// Also, I wish they said "have an established signal handler" or something like that rather than "handled" (above) or (worse) "caught" [`man 2 execve`](https://man7.org/linux/man-pages/man2/execve.2.html#:~:text=The%20dispositions%20of%20any%20signals%20that%20are%20being%20caught%20are%20reset%0A%20%20%20%20%20%20%20%20%20%20to%20the%20default%20(signal(7)).). Still, I think their meaning is pretty clear, especially from the existence of the phrase "caught, blocked, or ignored" [here on `man 7 signal`](https://man7.org/linux/man-pages/man7/signal.7.html#:~:text=caught%2C%20blocked%2C%20or%0A%20%20%20%20%20%20%20ignored).
// 2. This is much less clear to me. My gut tells me that it should be the whole `struct sigaction`, but the "disposition" is described as "determin[ing] how the process behaves when it is delivered the signal" and (confusingly) being one of "Term", "Ign", "Core", "Stop", or "Cont" - both right near the top of [`man 7 signal`](https://man7.org/linux/man-pages/man7/signal.7.html#DESCRIPTION). This difference should only reveal itself in things you can specify in `struct sigaction` with a handler that also have an effect with *no* handler, which as far as I can tell from [`man 2 sigaction`](https://man7.org/linux/man-pages/man2/sigaction.2.html) only includes `SA_NOCLDWAIT` and `SA_EXPOSE_TAGBITS`. A keyword I used to search on that page for whether specifying something affected things when not registering a handler was "establish", but I also had to use my noggin to rule out things like `sa_restorer` from mattering without a signal handler.
//
// TODO: Test with an ignored signal, and test with `SA_NOCLDWAIT`. See Discord for discussions.
fn main() {
    println!("Hello, world!");
}
