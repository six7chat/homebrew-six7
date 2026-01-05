class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.1 - powered by Korium 0.7.1"
  homepage "https://github.com/six7chat/homebrew-six7"
  url "https://github.com/six7chat/homebrew-six7/archive/refs/tags/v0.7.1.tar.gz"
  sha256 "93b5b6105829504485212d3c30548f848516c045f16a2ee4793bbf9ea165a5d0"
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
