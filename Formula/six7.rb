class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - leaner message format"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.6.25.tar.gz"
  sha256 "3ce8dfb5846277c75d89388e405b21be85333f980a8ad618ef25fec0363b6de6"
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
