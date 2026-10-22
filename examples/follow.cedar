@id("alice-follow-assistant")
permit(
    principal == Dekopon::Principal::"alice",
    action in [Dekopon::Action::"dekopon-provider-follow.stay",
               Dekopon::Action::"dekopon-provider-follow.leave"],
    resource == Dekopon::Provider::"dekopon-provider-follow"
)
when { context has agent && context.agent == "assistant"
    && context has via && context.via == "dekopon-gatewayd" };
