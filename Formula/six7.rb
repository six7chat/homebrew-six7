class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.53"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.53"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.53/six7-0.7.53-arm64-apple-darwin.tar.gz"
      sha256 "d8a5dd5909d3bdaadc1a8bdc8e50af71446b4ade294d36ef803afc077ab657c9"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
