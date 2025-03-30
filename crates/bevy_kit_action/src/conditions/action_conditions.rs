enum ActionConditionResult {
	/// The action can by emitted
	Enabled,
	/// The action can't be emitted and it won't be forwarded
	Disabled,
	/// The action won't be emitted but it will be forwarded through further mappings
	Bypass,
}
