class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.0"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.7.0.tar.gz"
  sha256 "9516d79fb929c6910ca07b8fa041e9edbd135cf1ce4d8833ceeda58b8997fa70"
  license "MIT"
  head "https://github.com/six7chat/homebrew-six7.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
